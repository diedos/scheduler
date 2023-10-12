<script lang="ts">
    import Task from 'components/Task.svelte';
    import Modal from 'components/Modal.svelte';
    import AddTask from 'components/AddTask.svelte';
    import CurrentTask from 'components/CurrentTask.svelte';

    export let data;

    let showAddTask = true;
</script>

<div class="flex flex-col max-w-7xl w-full space-y-2 mb-16 sm:px-4">
    <AddTask show />
    {#each data.todoTasks as task (task.id)}
        <Task {task} />
    {/each}
</div>
