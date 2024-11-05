package com.example.agent;

import sun.java2d.loops.TransformHelper;

import java.lang.instrument.Instrumentation;

public class MyAgent {
    // premain
    public static void premain(String agentArgs, Instrumentation inst) {
        startAgent("premain");
    }

    // attach
    public static void agentmain(String agentArgs, Instrumentation inst) {
        startAgent("agentmain");
    }

    // agent main
    public static void startAgent(String agentArgs) {
        Thread thread = new Thread(() -> {
            while (true) {
                System.out.println("Hello from JavaAgent!");
                try {
                    System.out.println("I'am sleepy. Sleep for five seconds.");
                    Thread.sleep(5000);
                } catch (InterruptedException e) {
                    System.out.println("Bye!");
                    e.printStackTrace();
                }
            }
        });
        thread.setDaemon(true);
        thread.start();
    }
}