package pkga

import "github.com/coldinke/exercise/goSpec/init_test/logger"

var Name = "Package A"

func init() {
	logger.LogInit("pkga", "init1")
}

func init() {
	logger.LogInit("pkga", "init2")
}

func init() {
	logger.LogInit("pkga", "init3")
}

func PublicFunc() string {
	return "This is from Package A"
}
