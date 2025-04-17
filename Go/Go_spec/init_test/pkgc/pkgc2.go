package pkgc

import (
	"github.com/coldinke/exercise/goSpec/init_test/logger"
)

func init() {
	logger.LogInit("pkgc", "init_file2")
}

var SecondaryVar = "Secondary variable from pkgc"

func SecondaryFunc() string {
	return "Secondary function from pkgc"
}
