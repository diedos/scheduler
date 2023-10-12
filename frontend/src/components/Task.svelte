<script lang="ts">
    import { enhance } from '$app/forms';
    import { fly, blur, slide } from 'svelte/transition';
    import Button from './Button.svelte';
    import Box from './Box.svelte';

    import { currentTask } from '../store';
    import type { TodoTask } from '../store';

    export let task: TodoTask;

    let deadlineIn: number;
    if (task.deadlineAt) {
        let deadlineDate = new Date(task.deadlineAt);
        let nowDate = new Date();
        deadlineIn = (deadlineDate.getTime() - nowDate.getTime()) / (1000 * 3600 * 24);

        console.log(deadlineIn);
    }
</script>

<a href="/tasks/{task.id}" class:opacity-40={task.completedAt} in:fly={{ y: 20 }} out:slide>
    <Box itemAlign="center">
        <div class="justify-between items-start flex flex-row w-2/6">
            <h4 class="flex text-md">{task.title}</h4>
        </div>
        <div class="w-2/6 px-4 flex-1">
            {task.content}
        </div>
        <div class="flex flex-1 flex-row justify-end items-center space-x-4">
            {#if deadlineIn}
                <span class="text-xs"
                    >Deadline in {deadlineIn.toFixed()} {deadlineIn === 1 ? 'day' : 'days'}
                </span>
            {/if}
        </div>
        <div class="flex flex-row ml-4 space-x-4">
            {#if $currentTask?.id === task.id}
                <Button color="neutral" disabled>In progress...</Button>
            {:else}
                <form method="POST" action="#" use:enhance>
                    <!-- TODO: save to backend-->
                    <Button color="neutral" on:click={() => currentTask.set(task)}
                        >Start task
                    </Button>
                </form>
            {/if}
            <form method="POST" action="?/complete" use:enhance>
                <input type="hidden" name="id" value={task.id} />
                <Button color="blue">
                    <span class="flex">Done</span>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        class="w-5 h-5 flex"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z"
                            clip-rule="evenodd"
                        />
                    </svg>
                </Button>
            </form>
        </div>
    </Box>
</a>
