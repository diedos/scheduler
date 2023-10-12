<script lang="ts">
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

<a
    class="absolute bottom-0 mb-8 self-center p-2 rounded-md bg-neutral-200 hover:bg-neutral-100 text-gray-800"
    href="/tasks">Show all tasks</a
>
