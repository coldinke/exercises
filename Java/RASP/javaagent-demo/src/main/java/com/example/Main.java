package com.example;

public class Main {
    public static void main(String[] args) throws InterruptedException {
        System.out.println("Main application started...");
        // 保持主程序运行
        while (true) {
            Thread.sleep(1000);
        }
    }
}
