var N = null;var sourcesIndex = {};
sourcesIndex["os"] = {"name":"","dirs":[{"name":"boards","files":["qemu.rs"]},{"name":"drivers","dirs":[{"name":"block","files":["mod.rs","sdcard.rs","virtio_blk.rs"]},{"name":"chardev","files":["mod.rs","ns16550a.rs"]}],"files":["mod.rs","plic.rs"]},{"name":"fs","files":["inode.rs","mod.rs","pipe.rs","stdio.rs"]},{"name":"mm","files":["address.rs","frame_allocator.rs","heap_allocator.rs","memory_set.rs","mod.rs","page_table.rs"]},{"name":"sync","files":["condvar.rs","ksync.rs","mod.rs","mutex.rs","semaphore.rs","up.rs"]},{"name":"syscall","files":["fs.rs","mod.rs","process.rs","sync.rs","thread.rs"]},{"name":"task","files":["action.rs","context.rs","kthread.rs","kthread_test.rs","manager.rs","mod.rs","pid.rs","pool.rs","processor.rs","signal.rs","switch.rs","task.rs"]},{"name":"trap","files":["context.rs","mod.rs"]}],"files":["config.rs","console.rs","lang_items.rs","main.rs","sbi.rs","timer.rs"]};
createSourceSidebar();
