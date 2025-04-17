package logger

import (
	"fmt"
	"sync"
	"time"
)

var (
	logs     []string
	initTime time.Time
	mu       sync.Mutex
)

func init() {
	initTime = time.Now()
	LogInit("logger", "init")
}

// LogInit 记录一个 init 函数被调用
func LogInit(pkg, funcName string) {
	mu.Lock()
	defer mu.Unlock()
	elapsed := time.Since(initTime).Nanoseconds()
	entry := fmt.Sprintf("%s.%s [%dns]", pkg, funcName, elapsed)
	logs = append(logs, entry)
}

// GetLogs 返回所有记录的 init 调用
func GetLogs() []string {
	mu.Lock()
	defer mu.Unlock()
	return append([]string{}, logs...)
}

// PrintLogs 打印所有记录的 init 调用
func PrintLogs() {
	mu.Lock()
	defer mu.Unlock()
	fmt.Println("\n=== Init Function Execution Order ===")
	for i, log := range logs {
		fmt.Printf("%d. %s\n", i+1, log)
	}
	fmt.Println("===================================")
}
