功能实现总结：
1、为TaskInfo实现系统调用次数的方法inc_syscall_times，
实现系统调用时长方法set_spend_time，
实现状态设置方法set_task_status。

2、在process.rs中实现get_time_kernel，用于获取系统调用时间，
实现方式和用户态的get_time保持一致。

3、在task/task.rs中为TaskControlBlock添加字段：
TaskInfo类型的task_info、usize类型的task_start_time，
用于标记任务执行开始时间。

4、在task/mod.rs中初始化tasks时为task_info、task_start_time执行初始化。
5、在task/mod.rs中为TaskManager实现系统调用次数累加inc_sys_call_times，
实现获取当前任务的任务信息get_current_task_info。
6、将系统调用次数累加方法和获取当前任务信息方法进行封装。
7、在syscall/mod.rs的syscall函数中调用系统调用次数累计函数inc_sys_call_times用以统计调用次数。
8、在syscall/process.rs中sys_task_info中添加获取当前任务信息的调用。

简答作业：
1、正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。
2、深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:

L40：刚进入 __restore 时，a0 代表了什么值。请指出 __restore 的两种使用情景。
a0代表
L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

ld t0, 32*8(sp)
ld t1, 33*8(sp)
ld t2, 2*8(sp)
csrw sstatus, t0
csrw sepc, t1
csrw sscratch, t2


在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

rcore源码和rCore-Camp-Guide-2024A

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。