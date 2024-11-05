package com.example.attacher;

import com.sun.tools.attach.VirtualMachine;
import com.sun.tools.attach.VirtualMachineDescriptor;
import java.util.List;

public class Attacher {
    public static void main(String[] args) {
        try {
            // 列出所有 Java 进程
            List<VirtualMachineDescriptor> vms = VirtualMachine.list();

            // 打印所有可用的 Java 进程
            System.out.println("Available Java processes:");
            for (VirtualMachineDescriptor vmd : vms) {
                System.out.println(vmd.id() + "\t" + vmd.displayName());
            }

            if (args.length == 0) {
                System.out.println("Please provide PID as argument");
                return;
            }

            // 获取目标进程 ID
            String pid = args[0];

            // 获取 agent jar 的路径
            String agentPath = args.length > 1 ? args[1] : "target/javaagent-demo-1.0-SNAPSHOT.jar";

            System.out.println("Attaching to process " + pid + " with agent " + agentPath);

            // 附加到目标 JVM
            VirtualMachine vm = VirtualMachine.attach(pid);

            System.out.println("Attached Success! Load agent...");


            // 加载 agent
            vm.loadAgent(agentPath);

            System.out.println("Load Success!");

            // 分离
            vm.detach();

            System.out.println("Agent attached successfully");

        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}