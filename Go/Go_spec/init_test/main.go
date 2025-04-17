package main

import (
	"fmt"

	"github.com/coldinke/exercise/goSpec/init_test/logger"
	_ "github.com/coldinke/exercise/goSpec/init_test/pkga" // 使用 _ 导入，表示我们只对其 init 函数感兴趣
	_ "github.com/coldinke/exercise/goSpec/init_test/pkgb"
	"github.com/coldinke/exercise/goSpec/init_test/pkgc"
)

var mainVar = "Main package variable"

func init() {
	logger.LogInit("main", "init1")
}

func init() {
	logger.LogInit("main", "init2")
}

func main() {
	fmt.Println("=== Testing Go Init Functions ===")
	fmt.Println("1. Multiple init functions in the same package")
	fmt.Println("2. Init functions across different packages")
	fmt.Println("3. Init functions along import dependency chain")

	fmt.Println("\nCalling pkgc.PublicFunc():", pkgc.PublicFunc())

	logger.PrintLogs()

	fmt.Println("\n=== Observations ===")
	fmt.Println("1. Within a package, init functions are executed in the order they appear in the source file")
	fmt.Println("2. For imports, the init functions of imported packages are executed before the importing package")
	fmt.Println("3. The dependency chain is resolved recursively: pkga -> pkgb -> pkgc -> main")
	fmt.Println("4. Package-level variables are initialized before any init functions in that package")
}
