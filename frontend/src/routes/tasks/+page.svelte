<script lang="ts">
    import Task from 'components/Task.svelte';
    import Modal from 'components/Modal.svelte';
    import AddTask from 'components/AddTask.svelte';
    import CurrentTask from 'components/CurrentTask.svelte';

    export let data;

    let showAddTask = true;
</script>

<div class="flex flex-col max-w-7xl w-full space-y-2">
    <AddTask show />
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
