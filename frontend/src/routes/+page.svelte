<script lang="ts">
    import Task from '../components/Task.svelte';
    import Modal from '../components/Modal.svelte';

    export let data;

    let showModal = false;

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'n') {
            showModal = true;
        }
    }
</script>

<svelte:window on:keydown={handleKeydown} />

<button on:click={() => (showModal = true)}> show modal </button>

<Modal bind:showModal />

<div class="flex flex-row flex-wrap">
    {#each data.todoTasks as task}
        <Task
            title={task.title}
            content={task.content}
            createdAt={task.createdAt}
            completed={task.completed}
        />
    {/each}
</div>
