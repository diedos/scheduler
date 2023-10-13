<script lang="ts">
    import { currentTask } from '../store.js';

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

<svelte:head>
    <title>{$currentTask?.id ? $currentTask?.title : 'Not working on any task'}</title>
</svelte:head>

<!-- <svelte:window on:keydown={handleKeydown} /> -->

<a
    class="absolute bottom-0 mb-8 self-center p-4 rounded-md bg-neutral-200 hover:bg-neutral-100 text-gray-800"
    href="/tasks">Show all tasks</a
>
