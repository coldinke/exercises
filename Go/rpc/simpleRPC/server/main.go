package main

import (
	"log"
	"net"
	"net/http"
	"net/rpc"
	"time"
)

type Args struct{}
type TimeServer int64

func (t *TimeServer) GiveServerTime(args *Args, reply *int64) error {
	*reply = time.Now().Unix()
	return nil
}

func main() {
	timeserver := new(TimeServer)
	rpc.Register(timeserver)
	rpc.HandleHTTP()
	l, e := net.Listen("tcp", ":2233")
	if e != nil {
		log.Fatal("listen error: ", e)
	}
	http.Serve(l, nil)
}
