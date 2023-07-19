<script lang="ts">
    import { enhance } from '$app/forms';
    import { fly, blur, slide } from 'svelte/transition';

    export let id: number;
    export let title: string;
    export let content: string;
    export let createdAt: string;
    export let completedAt: string;
    export let deadlineAt: string;

    let deadlineIn: number;
    if (deadlineAt) {
        let deadlineDate = new Date(deadlineAt);
        let nowDate = new Date();
        deadlineIn = (deadlineDate.getTime() - nowDate.getTime()) / (1000 * 3600 * 24);
    }
</script>

<a
    href="/tasks/{id}"
    class="bg-gray-50 border-t-white text-gray-800 dark:bg-slate-800 flex flex-row w-full border-t dark:border-t-slate-600 rounded-xl dark:text-slate-300 overflow-hidden items-center px-4 py-2"
    class:opacity-40={completedAt}
    in:fly={{ y: 20 }}
    out:slide
>
    <header class="justify-between items-center flex flex-row w-2/6">
        <h4 class="flex text-md">{title}</h4>
    </header>
    <section class="w-2/6 py-2 px-4 flex-1">
        {content}
    </section>
    <footer class="flex flex-1 flex-row justify-end items-center space-x-4">
        {#if deadlineIn}
            <span class="text-xs"
                >Deadline in {deadlineIn.toFixed()} {deadlineIn === 1 ? 'day' : 'days'}</span
            >
        {/if}
    </footer>
    <form method="POST" action="?/complete" use:enhance>
        <input type="hidden" name="id" value={id} />
        <button
            type="submit"
            class="aspect-square h-auto w-auto rounded-md p-1 bg-emerald-600 text-gray-50"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 20 20"
                fill="currentColor"
                class="w-5 h-5"
            >
                <path
                    fill-rule="evenodd"
                    d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z"
                    clip-rule="evenodd"
                />
            </svg>
        </button>
    </form>
</a>
