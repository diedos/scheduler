<script lang="ts">
    import Task from 'components/Task.svelte';
    import Modal from 'components/Modal.svelte';
    import AddTask from 'components/AddTask.svelte';
    import CurrentTask from 'components/CurrentTask.svelte';

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

<CurrentTask />
