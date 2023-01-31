package main

import (
	"fmt"
	"math/rand"
)

// 写代码实现两个 goroutine，其中一个产生随机数并写入到 go channel 中，
// 另外一个从 channel 中读取数字并打印到标准输出。最终输出五个随机数。
// https://studygolang.com/articles/18942

func main() {
	ch := make(chan int)
	done := make(chan bool)

	go func() {
		for {
			select {
			case ch <- rand.Intn(5):
			case <-done:
				return
			default:
			}
		}
	}()

	go func() {
		for i := 0; i < 5; i++ {
			fmt.Println(<-ch)
		}
		done <- true
		return
	}()

	<-done
}

//func main() {
//	ch := make(chan int)
//
//	for i := 0; i < 5; i++ {
//		go func(n int) {
//			ch <- n
//		}(i)
//
//		go func() {
//			fmt.Println(<-ch)
//		}()
//	}
//
//	time.Sleep(2 * time.Second)
//}
