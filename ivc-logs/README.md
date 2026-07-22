# LoongArch IVC Clock Sync Board Test

- Date: 2026-07-22
- Board: Loongson 3A6000
- Root Linux: 6.11.6
- Non-root Linux: 6.13.7
- IVC channel: 0
- IVC interrupt: HWI 6
- Counter frequency: 100 MHz
- EFI SHA-256: `db0de14517cc3f17f2dd113eca6904fef93a2e0038b38baec8a110ba0f95ca0c`
- Test load: 5 runs, 100 samples per run, 500 samples total

## Fixed Results

| Run | Samples | Avg offset (cycles) | Avg offset (us) | Avg latency (cycles) | Avg latency (us) | Min offset | Max offset | Status |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 01 | 100 | -17650251033.870 | -176502510.339 | 12101362.230 | 121013.622 | -17662253786 | -17614904693 | PASS |
| 02 | 100 | -17647268315.810 | -176472683.158 | 15083962.770 | 150839.628 | -17661915402 | -17613901602 | PASS |
| 03 | 100 | -17648891422.670 | -176488914.227 | 13460846.420 | 134608.464 | -17662273880 | -17614904752 | PASS |
| 04 | 100 | -17650826959.430 | -176508269.594 | 11525318.480 | 115253.185 | -17662285683 | -17620942388 | PASS |
| 05 | 100 | -17651951031.410 | -176519510.314 | 10401265.260 | 104012.653 | -17662253521 | -17626415685 | PASS |
| Overall | 500 | -17649837752.638 | -176498377.526 | 12514551.032 | 125145.510 | -17662285683 | -17613901602 | PASS |

All averages above were recalculated from the raw per-sample CSV rows rather
than copied from the program summary line.

## Regression Evidence

Before re-arming the IVC virtual interrupt line for each doorbell, runs 01-03
completed, run 04 timed out at sequence 58, and runs 05-06 timed out at
sequence 1. Those raw logs are retained as `pre-fix-run-*.log`.

After changing each notification to deassert then assert HWI 6, all five
consecutive runs completed in one linux1 lifetime without restarting the zone.

## Files

- `fixed-run-01.log` through `fixed-run-05.log`: raw successful test output.
- `fixed-status.log`: EFI identity and command return codes.
- `fixed-slave.log`: linux1-side kernel, module-load, and device-node evidence.
- `pre-fix-run-01.log` through `pre-fix-run-06.log`: pre-fix reliability evidence.
- `pre-fix-status.log`: pre-fix command return codes.
- `board-uart.log`: board UART capture for the fixed test boot.
- `summary.csv`: machine-readable fixed-result table.
- `SHA256SUMS`: hashes for the raw logs and summary.
