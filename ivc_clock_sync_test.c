// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2026 Syswonder
 */
#include "ivc.h"

#include <errno.h>
#include <fcntl.h>
#include <poll.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>

#define PTP_MAGIC 0x505450495643ULL
#define DEFAULT_SAMPLES 1000U
#define DEFAULT_TIMEOUT_MS 2000
#define REQUEST_SPIN_LIMIT 1000000U
#define LOONGARCH_CPUCFG_CC_FREQ 0x4
#define LOONGARCH_CPUCFG_CC_MUL_DIV 0x5

struct ptp_ivc_msg {
    volatile uint64_t magic;
    volatile uint32_t seq;
    volatile uint32_t ready;
    volatile uint64_t t1;
    volatile uint64_t t2;
    volatile uint64_t t3;
    volatile uint64_t t4;
};

static volatile struct ptp_ivc_msg *out_msg;
static volatile struct ptp_ivc_msg *in_msg;
static ivc_cttable_t *tb;
static uint32_t peer_target_id;

static inline uint64_t read_counter(void)
{
#if !defined(__loongarch64)
#error "ivc_clock_sync_test is for LoongArch64/3A6000"
#endif
    uint64_t v;
    uint64_t tmp;
    asm volatile("rdtime.d %0, %1" : "=r"(v), "=r"(tmp));
    return v;
}

static inline uint32_t read_loongarch_cpucfg(uint32_t index)
{
    uint32_t v;
    asm volatile("cpucfg %0, %1" : "=r"(v) : "r"(index));
    return v;
}

static inline uint64_t read_counter_freq(void)
{
    uint64_t cc_freq = read_loongarch_cpucfg(LOONGARCH_CPUCFG_CC_FREQ);
    uint32_t mul_div = read_loongarch_cpucfg(LOONGARCH_CPUCFG_CC_MUL_DIV);
    uint64_t cc_mul = mul_div & 0xffffU;
    uint64_t cc_div = (mul_div >> 16) & 0xffffU;

    if (cc_div == 0)
        return 0;
    return (cc_freq * cc_mul) / cc_div;
}

static inline void mb(void)
{
    __sync_synchronize();
}

static const char *counter_name(void)
{
    return "stable-counter";
}

static int open_dev(void)
{
    int fd = open("/dev/hivc0", O_RDWR);
    if (fd < 0) {
        perror("open /dev/hivc0");
        exit(1);
    }
    return fd;
}

static void usage(const char *prog)
{
    printf("Usage: %s master|slave [samples]\n", prog);
}

static void map_like_ivc_demo(int fd, int is_master)
{
    void *tb_virt;
    unsigned long long offset = 0x1000;

    tb_virt = mmap(NULL, 0x1000, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (tb_virt == MAP_FAILED) {
        perror("mmap control table");
        exit(1);
    }
    tb = (ivc_cttable_t *)tb_virt;

    if (tb->out_sec_size < sizeof(struct ptp_ivc_msg)) {
        fprintf(stderr, "out_sec_size %u is smaller than ptp message %zu\n",
                tb->out_sec_size, sizeof(struct ptp_ivc_msg));
        exit(1);
    }

    if (tb->max_peers != 2) {
        fprintf(stderr, "this test expects exactly 2 peers, got %u\n",
                tb->max_peers);
        exit(1);
    }
    peer_target_id = tb->peer_id ^ 1U;

    printf("ivc_id=%u max_peers=%u peer_id=%u target_peer=%u "
           "out_sec_size=%u counter=%s freq=%llu\n",
           tb->ivc_id, tb->max_peers, tb->peer_id, peer_target_id,
           tb->out_sec_size, counter_name(),
           (unsigned long long)read_counter_freq());

    if (is_master) {
        out_msg = mmap(NULL, tb->out_sec_size, PROT_READ | PROT_WRITE,
                       MAP_SHARED, fd, offset);
        if (out_msg == MAP_FAILED) {
            perror("mmap master out");
            exit(1);
        }
        offset += tb->out_sec_size;
        in_msg = mmap(NULL, tb->out_sec_size, PROT_READ, MAP_SHARED, fd,
                      offset);
        if (in_msg == MAP_FAILED) {
            perror("mmap master in");
            exit(1);
        }
    } else {
        in_msg = mmap(NULL, tb->out_sec_size, PROT_READ, MAP_SHARED, fd,
                      offset);
        if (in_msg == MAP_FAILED) {
            perror("mmap slave in");
            exit(1);
        }
        offset += tb->out_sec_size;
        out_msg = mmap(NULL, tb->out_sec_size, PROT_READ | PROT_WRITE,
                       MAP_SHARED, fd, offset);
        if (out_msg == MAP_FAILED) {
            perror("mmap slave out");
            exit(1);
        }
    }
}

static void wait_for_reply(struct pollfd *pfd, uint32_t seq)
{
    while (in_msg->magic != PTP_MAGIC || in_msg->seq != seq ||
           in_msg->ready == 0) {
        int ret = poll(pfd, 1, DEFAULT_TIMEOUT_MS);
        if (ret < 0) {
            if (errno == EINTR)
                continue;
            perror("poll");
            exit(1);
        }
        if (ret == 0) {
            fprintf(stderr, "timeout waiting for seq %u\n", seq);
            exit(1);
        }
    }
}

static void run_master(int fd, unsigned int samples)
{
    struct pollfd pfd = {.fd = fd, .events = POLLIN};
    long double sum_offset = 0.0;
    long double sum_latency = 0.0;
    uint64_t freq = read_counter_freq();
    int64_t min_offset = INT64_MAX;
    int64_t max_offset = INT64_MIN;

    if (freq == 0) {
        fprintf(stderr, "counter frequency is zero\n");
        exit(1);
    }

    memset((void *)out_msg, 0, sizeof(*out_msg));
    mb();

    printf("seq,t1,t2,t3,t4,offset_cycles,latency_cycles,offset_us,latency_us\n");
    for (uint32_t seq = 1; seq <= samples; seq++) {
        out_msg->magic = PTP_MAGIC;
        out_msg->seq = seq;
        out_msg->ready = 0;
        out_msg->t2 = 0;
        out_msg->t3 = 0;
        mb();

        uint64_t t1 = read_counter();
        out_msg->t1 = t1;
        out_msg->ready = 1;
        mb();
        tb->ipi_invoke = peer_target_id;

        wait_for_reply(&pfd, seq);
        uint64_t t4 = read_counter();

        uint64_t t2 = in_msg->t2;
        uint64_t t3 = in_msg->t3;
        int64_t offset = ((int64_t)(t2 - t1) + (int64_t)(t3 - t4)) / 2;
        int64_t latency = ((int64_t)(t4 - t1) - (int64_t)(t3 - t2)) / 2;

        if (offset < min_offset)
            min_offset = offset;
        if (offset > max_offset)
            max_offset = offset;
        sum_offset += offset;
        sum_latency += latency;

        printf("%u,%llu,%llu,%llu,%llu,%lld,%lld,%.3Lf,%.3Lf\n", seq,
               (unsigned long long)t1, (unsigned long long)t2,
               (unsigned long long)t3, (unsigned long long)t4,
               (long long)offset, (long long)latency,
               ((long double)offset * 1000000.0L) / freq,
               ((long double)latency * 1000000.0L) / freq);

        out_msg->ready = 0;
        mb();
        usleep(1000);
    }

    printf("summary: samples=%u offset_avg_cycles=%.3Lf "
           "offset_avg_us=%.3Lf latency_avg_cycles=%.3Lf "
           "offset_min=%lld offset_max=%lld\n",
           samples, sum_offset / samples,
           ((sum_offset / samples) * 1000000.0L) / freq,
           sum_latency / samples,
           (long long)min_offset, (long long)max_offset);
}

static void run_slave(int fd)
{
    struct pollfd pfd = {.fd = fd, .events = POLLIN};
    uint32_t last_seq = 0;

    memset((void *)out_msg, 0, sizeof(*out_msg));
    mb();
    printf("slave ready\n");

    for (;;) {
        int ret = poll(&pfd, 1, -1);
        if (ret < 0) {
            if (errno == EINTR)
                continue;
            perror("poll");
            exit(1);
        }

        for (unsigned int i = 0;
             (in_msg->magic != PTP_MAGIC || in_msg->ready == 0 ||
              in_msg->seq == last_seq) &&
             i < REQUEST_SPIN_LIMIT;
             i++) {
            mb();
        }

        if (in_msg->magic != PTP_MAGIC || in_msg->ready == 0 ||
            in_msg->seq == last_seq) {
            fprintf(stderr,
                    "slave woke but no new request: magic=0x%llx ready=%u "
                    "seq=%u last_seq=%u\n",
                    (unsigned long long)in_msg->magic, in_msg->ready,
                    in_msg->seq, last_seq);
            continue;
        }

        uint32_t seq = in_msg->seq;
        uint64_t t2 = read_counter();

        out_msg->magic = PTP_MAGIC;
        out_msg->seq = seq;
        out_msg->ready = 0;
        out_msg->t1 = in_msg->t1;
        out_msg->t2 = t2;
        mb();
        out_msg->t3 = read_counter();
        out_msg->ready = 1;
        mb();
        tb->ipi_invoke = peer_target_id;

        last_seq = seq;
    }
}

int main(int argc, char *argv[])
{
    int is_master;
    unsigned int samples = DEFAULT_SAMPLES;
    int fd;

    if (argc < 2 || argc > 3) {
        usage(argv[0]);
        return 2;
    }
    if (strcmp(argv[1], "master") == 0) {
        is_master = 1;
    } else if (strcmp(argv[1], "slave") == 0) {
        is_master = 0;
    } else {
        usage(argv[0]);
        return 2;
    }
    if (argc == 3)
        samples = (unsigned int)strtoul(argv[2], NULL, 0);
    if (samples == 0) {
        usage(argv[0]);
        return 2;
    }

    fd = open_dev();
    map_like_ivc_demo(fd, is_master);

    if (is_master)
        run_master(fd, samples);
    else
        run_slave(fd);

    close(fd);
    return 0;
}
