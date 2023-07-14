<script lang="ts">
    export let title: string;
    export let content: string;
    export let createdAt: string;
    export let completed: boolean;
</script>

<div
    class="bg-slate-800 flex aspect-square flex-col border-t border-l border-t-slate-600 border-l-slate-600 rounded-xl text-slate-300 overflow-hidden"
    class:opacity-40={completed}
>
    <header
        class="bg-slate-700 justify-between items-center flex flex-row w-full py-2 px-4 border-b-2 border-b-emerald-400"
    >
        <h4 class="flex text-xl">{title}</h4>
        <button class="flex align-middle"
            ><svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="w-6 h-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15"
                />
            </svg>
        </button>
    </header>
    <section class="w-full py-2 px-4 text-slate-400 flex-1">
        {content}
    </section>
    <footer class="flex flex-row justify-between pt-2 pb-4 px-4">
        <span class="text-xs text-slate-400">{createdAt}</span>
        <span class="text-xs text-slate-400"> Deadline in 2 days</span>
    </footer>
</div>
