package pkgc

import (
	"github.com/coldinke/exercise/goSpec/init_test/logger"
	"github.com/coldinke/exercise/goSpec/init_test/pkgb"
)

var Name = "Package C"

var importedPkgName = pkgb.Name

func init() {
	logger.LogInit("pkgc", "init1")
}

func init() {
	logger.LogInit("pkgc", "init2")
}

func PublicFunc() string {
	return "This is from Package C, calling: " + pkgb.PublicFunc()
}
