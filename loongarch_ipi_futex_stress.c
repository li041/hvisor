// SPDX-License-Identifier: GPL-2.0-only
/*
 * Self-contained LoongArch/Linux cross-CPU IPI stress test.
 *
 * Build:
 *   gcc -O2 -Wall -Wextra -pthread loongarch_ipi_futex_stress.c -o loongarch_ipi_futex_stress
 *
 * Run examples:
 *   ./loongarch_ipi_futex_stress
 *   ./loongarch_ipi_futex_stress -a 0 -b 1 -n 1000000
 *   ./loongarch_ipi_futex_stress -a 0 -b 4 -n 1000000 -t 1000
 *
 * The test pins two threads to two CPUs and uses futex wakeups to ping-pong.
 * On SMP Linux this path normally causes cross-CPU reschedule/wakeup IPIs.
 * Lost virtual IPIs usually show up as timeout, hang, or hvisor/kernel logs.
 */
#define _GNU_SOURCE

#include <errno.h>
#include <limits.h>
#include <pthread.h>
#include <sched.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/syscall.h>
#include <time.h>
#include <unistd.h>
#include <linux/futex.h>

static volatile int token;
static volatile int stop_flag;
static int cpu_a = 0;
static int cpu_b = 1;
static uint64_t rounds = 1000000;
static int timeout_ms = 2000;

struct worker_arg {
    int cpu;
    int wait_value;
    int next_value;
    uint64_t count;
    uint64_t timeouts;
};

static void die(const char *msg)
{
    perror(msg);
    exit(1);
}

static long futex_wait(volatile int *addr, int val, int timeout)
{
    struct timespec ts;
    ts.tv_sec = timeout / 1000;
    ts.tv_nsec = (long)(timeout % 1000) * 1000000L;
    return syscall(SYS_futex, addr, FUTEX_WAIT, val, &ts, NULL, 0);
}

static long futex_wake(volatile int *addr)
{
    return syscall(SYS_futex, addr, FUTEX_WAKE, 1, NULL, NULL, 0);
}

static void pin_to_cpu(int cpu)
{
    cpu_set_t set;
    CPU_ZERO(&set);
    CPU_SET(cpu, &set);
    if (sched_setaffinity(0, sizeof(set), &set) != 0)
        die("sched_setaffinity");
}

static uint64_t now_ns(void)
{
    struct timespec ts;
    if (clock_gettime(CLOCK_MONOTONIC, &ts) != 0)
        die("clock_gettime");
    return (uint64_t)ts.tv_sec * 1000000000ULL + (uint64_t)ts.tv_nsec;
}

static void *worker(void *data)
{
    struct worker_arg *arg = data;
    pin_to_cpu(arg->cpu);

    while (!stop_flag) {
        while (!stop_flag && __atomic_load_n(&token, __ATOMIC_ACQUIRE) != arg->wait_value) {
            long ret = futex_wait(&token, arg->next_value, timeout_ms);
            if (ret < 0 && errno == ETIMEDOUT) {
                arg->timeouts++;
                fprintf(stderr,
                        "timeout on cpu %d: token=%d expected=%d after count=%llu\n",
                        arg->cpu,
                        __atomic_load_n(&token, __ATOMIC_RELAXED),
                        arg->wait_value,
                        (unsigned long long)arg->count);
                stop_flag = 1;
                futex_wake(&token);
                return NULL;
            }
            if (ret < 0 && errno != EAGAIN && errno != EINTR)
                die("futex_wait");
        }

        if (stop_flag)
            break;

        arg->count++;
        __atomic_store_n(&token, arg->next_value, __ATOMIC_RELEASE);
        futex_wake(&token);

        if (arg->wait_value == 1 && arg->count >= rounds) {
            stop_flag = 1;
            futex_wake(&token);
            break;
        }
    }

    return NULL;
}

static long parse_long(const char *s, const char *name)
{
    char *end = NULL;
    errno = 0;
    long v = strtol(s, &end, 0);
    if (errno || !end || *end != '\0') {
        fprintf(stderr, "invalid %s: %s\n", name, s);
        exit(2);
    }
    return v;
}

static void usage(const char *prog)
{
    fprintf(stderr,
            "Usage: %s [-a cpu] [-b cpu] [-n rounds] [-t timeout_ms]\n"
            "Default: -a 0 -b 1 -n 1000000 -t 2000\n",
            prog);
}

int main(int argc, char **argv)
{
    int opt;
    int ncpu = (int)sysconf(_SC_NPROCESSORS_ONLN);
    pthread_t ta, tb;
    struct worker_arg wa;
    struct worker_arg wb;
    uint64_t start;
    uint64_t end;

    while ((opt = getopt(argc, argv, "a:b:n:t:h")) != -1) {
        switch (opt) {
        case 'a':
            cpu_a = (int)parse_long(optarg, "cpu_a");
            break;
        case 'b':
            cpu_b = (int)parse_long(optarg, "cpu_b");
            break;
        case 'n':
            rounds = (uint64_t)parse_long(optarg, "rounds");
            break;
        case 't':
            timeout_ms = (int)parse_long(optarg, "timeout_ms");
            break;
        case 'h':
        default:
            usage(argv[0]);
            return opt == 'h' ? 0 : 2;
        }
    }

    if (ncpu <= 0)
        die("sysconf(_SC_NPROCESSORS_ONLN)");
    if (cpu_a < 0 || cpu_b < 0 || cpu_a >= ncpu || cpu_b >= ncpu || cpu_a == cpu_b) {
        fprintf(stderr, "bad CPU selection: cpu_a=%d cpu_b=%d online_cpus=%d\n",
                cpu_a, cpu_b, ncpu);
        return 2;
    }
    if (rounds == 0 || rounds > (uint64_t)LLONG_MAX || timeout_ms <= 0) {
        usage(argv[0]);
        return 2;
    }

    memset(&wa, 0, sizeof(wa));
    memset(&wb, 0, sizeof(wb));
    wa.cpu = cpu_a;
    wa.wait_value = 0;
    wa.next_value = 1;
    wb.cpu = cpu_b;
    wb.wait_value = 1;
    wb.next_value = 0;

    token = 0;
    stop_flag = 0;

    printf("ipi futex stress: cpu_a=%d cpu_b=%d rounds=%llu timeout_ms=%d online_cpus=%d\n",
           cpu_a, cpu_b, (unsigned long long)rounds, timeout_ms, ncpu);

    start = now_ns();
    if (pthread_create(&ta, NULL, worker, &wa) != 0)
        die("pthread_create cpu_a");
    if (pthread_create(&tb, NULL, worker, &wb) != 0)
        die("pthread_create cpu_b");

    pthread_join(ta, NULL);
    pthread_join(tb, NULL);
    end = now_ns();

    double sec = (double)(end - start) / 1000000000.0;
    printf("done: cpu%d_count=%llu cpu%d_count=%llu timeouts=%llu/%llu elapsed=%.6f sec rate=%.0f roundtrips/sec\n",
           cpu_a, (unsigned long long)wa.count,
           cpu_b, (unsigned long long)wb.count,
           (unsigned long long)wa.timeouts,
           (unsigned long long)wb.timeouts,
           sec,
           sec > 0.0 ? (double)wb.count / sec : 0.0);

    if (wa.timeouts || wb.timeouts) {
        fprintf(stderr, "FAIL: futex wakeup timed out; check hvisor/kernel IPI logs\n");
        return 1;
    }
    if (wb.count < rounds) {
        fprintf(stderr, "FAIL: incomplete rounds: got %llu expected %llu\n",
                (unsigned long long)wb.count, (unsigned long long)rounds);
        return 1;
    }

    printf("PASS\n");
    return 0;
}
