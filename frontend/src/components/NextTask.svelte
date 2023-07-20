<script lang="ts">
    import { enhance } from '$app/forms';
    import { fly, blur, slide } from 'svelte/transition';

    export let id: number;
    export let title: string;
    export let content: string;
    export let createdAt: string;
    export let completed: string;
    export let deadlineAt: string;

    let deadlineIn: number;
    if (deadlineAt) {
        let deadlineDate = new Date(deadlineAt);
        let nowDate = new Date();
        deadlineIn = (deadlineDate.getTime() - nowDate.getTime()) / (1000 * 3600 * 24);
    }
</script>

<div
    class="bg-gray-50 text-gray-800 dark:bg-slate-800 flex flex-col dark:text-slate-300 overflow-hidden border-2 border-emerald-400 shadow-md w-1/2 p-4 space-y-4"
    class:opacity-40={completed}
    in:fly={{ y: 20 }}
    out:slide
>
    <header class="justify-between items-center flex flex-row">
        <h3 class="flex text-xl">Task</h3>
    </header>
    <section class="flex flex-1">
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Impedit saepe dolorem, facere sit
        blanditiis in explicabo perferendis dolores placeat reprehenderit incidunt voluptatibus.
        Beatae, laboriosam iure enim repellat incidunt aperiam facilis.
    </section>
</div>
