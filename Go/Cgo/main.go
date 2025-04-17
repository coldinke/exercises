package main

/*
#include <time.h>
#include <stdlib.h>
#include <sys/sysinfo.h>

unsigned long long getMachineStartTimeNano() {
	struct sysinfo s_info;
	int error = sysinfo(&s_info);
	if (error != 0) {
		return 0;
	}

	time_t current_time = time(NULL);

	time_t boot_time = current_time - s_info.uptime;
	unsigned long long boot_time_nano = (unsigned long long)boot_time * 1000000000ULL;

	return boot_time_nano;
}
*/
import "C"
import "fmt"

func Randome() int {
	return int(C.random())
}

func Seed(i int) {
	C.srandom(C.uint(i))
}

func GetMachineStartTimeNano() uint64 {
	return uint64(C.getMachineStartTimeNano())
}

func main() {
	startTime := GetMachineStartTimeNano()
	fmt.Printf("Machine start time (nanoseconds since epoch): %d\n", startTime)

	Seed(10)

	fmt.Printf("Random number form C: %d\n", Randome())
}
