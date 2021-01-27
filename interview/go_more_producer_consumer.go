package main

import (
	"fmt"
	"math/rand"
	"time"
)

// go实现多个生产者、多个消费者

func producer(name string, buf chan<- int) {
	for {
		num := rand.Intn(5)
		fmt.Println("producer: ", name, "producing: ", num)
		buf <- num
		time.Sleep(time.Second)
	}
}

func consumer(name string, buf <-chan int) {
	for {
		num := <- buf
		fmt.Println("consumer: ", name, "consuming: ", num)
	}
}

//func producer_for(name string, buf chan<- int) {
//	for i := 0; i < 5; i++ {
//		fmt.Println("producer: ", name, "producing: ", i)
//		buf <- i
//		time.Sleep(time.Second)
//	}
//}
//
//func consumer_for(name string, buf <-chan int, msg chan<- string) {
//	for {
//		num := <- buf
//		fmt.Println("consumer: ", name, "consuming: ", num)
//	}
//}

func main() {
	bufChan := make(chan int, 100)
	//msgChan := make(chan int)

	// 开启多个生产者
	go producer("p1", bufChan)
	go producer("p2", bufChan)
	go producer("p3", bufChan)

	// 开启多个消费者
	go consumer("c1", bufChan)
	go consumer("c2", bufChan)

	time.Sleep(10 * time.Second)
}
