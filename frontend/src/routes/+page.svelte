<script lang="ts">
    import Task from 'components/Task.svelte';
    import Modal from 'components/Modal.svelte';
    import AddTask from 'components/AddTask.svelte';

    export let data;

    let showAddTask = true;

    // function handleKeydown(event: KeyboardEvent) {
    //     if (event.key === 'n') {
    //         showAddTask = !showAddTask;
    //     }
    //     if (event.key === 'd') {
    //         window.document.body.classList.toggle('dark');
    //     }
    // }
</script>

<!-- <svelte:window on:keydown={handleKeydown} /> -->

<AddTask show={showAddTask} />

<div class="space-y-2">
    {#each data.todoTasks as task (task.id)}
        <Task
            id={task.id}
            title={task.title}
            content={task.content || ''}
            createdAt={task.createdAt}
            completedAt={task.completedAt}
            deadlineAt={task.deadlineAt}
        />
    {/each}
</div>
