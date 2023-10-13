<script lang="ts">
    import Task from 'components/Task.svelte';
    import Modal from 'components/Modal.svelte';
    import AddTask from 'components/AddTask.svelte';
    import CurrentTask from 'components/CurrentTask.svelte';

    import { fade } from 'svelte/transition';

    export let data;

    let showAddTask = true;
</script>

<svelte:head>
    <title>All tasks</title>
</svelte:head>

<div class="flex flex-col max-w-7xl w-full space-y-2 mb-16 sm:px-4 max-lg:p-4" in:fade>
    <AddTask show />
    {#each data.todoTasks as task (task.id)}
        <Task {task} />
    {/each}
</div>
