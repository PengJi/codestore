package main

import (
	"fmt"
	"math/rand"
	"time"
)

// go 实现单个生产者、单个消费者

func producer(name string, buf chan<- int) {
	for i := 0; i < 5; i++ {
		fmt.Println("producer: ", name, "producing: ", i)
		buf <- i
		time.Sleep(time.Second)
	}
}

func consumer(name string, buf <-chan int) {
	for {
		num := <- buf
		fmt.Println("consumer: ", name, "consuming: ", num)
	}
}

//func productor(channel chan<- string) {
//	for {
//		channel <- fmt.Sprintf("%v", rand.Float64())
//		time.Sleep(time.Second * time.Duration(1))
//	}
//}
//
//func customer(channel <-chan string) {
//	for {
//		message := <-channel // 此处会阻塞, 如果信道中没有数据的话
//		fmt.Println(message)
//	}
//}

func main() {
	bufChan := make(chan int, 10)
	//msgChan := make(chan string)

	go producer("p1", bufChan)
	go consumer("c1", bufChan)

	time.Sleep(10 * time.Second)

	//channel := make(chan string, 5) // 定义带有5个缓冲区的信道(当然可以是其他数字)
	//go productor(channel) // 将 productor 函数交给协程处理, 产生的结果传入信道中
	//customer(channel) // 主线程从信道中取数据
}
