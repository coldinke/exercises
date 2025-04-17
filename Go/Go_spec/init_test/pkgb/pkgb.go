package pkgb

import (
	"github.com/coldinke/exercise/goSpec/init_test/logger"
	"github.com/coldinke/exercise/goSpec/init_test/pkga"
)

var Name = "Package B"

func init() {
	logger.LogInit("pkgb", "init1")
}

func init() {
	logger.LogInit("pkgb", "init2")
}

func PublicFunc() string {
	return "This is from Package B, calling: " + pkga.PublicFunc()
}
