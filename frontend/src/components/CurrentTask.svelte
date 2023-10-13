<script lang="ts">
    import { slide, scale } from 'svelte/transition';
    import { enhance } from '$app/forms';

    import { currentTask } from '../store';
    import Button from './Button.svelte';

    export let banner = false;
</script>

<div
    class={banner
        ? 'transition-all duration-500 lg:mb-4 w-full max-w-full sticky top-0 mt-0 backdrop-blur-md translate-y-0 z-10'
        : 'transition-all duration-500 max-w-7xl w-full lg:absolute lg:top-1/2 lg:-translate-y-1/2'}
>
    <div
        class="flex flex-col w-full max-lg:flex-col-reverse lg:flex-row-reverse justify-center transition-all duration-500"
        class:sm:mt-0={banner}
        class:sm:ml-0={banner}
        class:sm:mr-0={banner}
        class:max-lg:p-4={!banner}
    >
        {#if !banner}
            <div
                class="flex max-lg:flex-row lg:flex-col max-lg:m-4 max-lg:space-x-4 lg:space-y-4 lg:ml-4 max-lg:mb-4 max-sm:mb-2 grow lg:w-1/6 overflow-hidden"
            >
                <Button color="neutral" disabled wide large>Take a break</Button>
                <form
                    method="POST"
                    action="?/complete"
                    use:enhance={({ formElement, formData, action, cancel, submitter }) => {
                        return async ({ result, update }) => {
                            currentTask.set(result.data.nextTask);
                            update();
                        };
                    }}
                >
                    <input type="hidden" name="id" value={$currentTask?.id} />
                    <Button color="blue" disabled={!$currentTask?.id} wide large>
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
        {/if}
        {#if banner}
            <a href="/" class="w-full h-full flex items-center justify-center">
                <div
                    class={banner
                        ? 'flex flex-col bg-gradient-to-tr from-sky-translucent to-emerald-translucent items-center justify-center p-4 max-lg:w-full lg:w-full transition-all duration-500'
                        : 'flex flex-col bg-gradient-to-tr from-sky-500 to-emerald-500 items-center justify-center rounded-3xl p-4 max-lg:w-full lg:w-5/6 transition-all duration-500'}
                >
                    <div class="w-full max-w-7xl">
                        <div
                            class="flex flex-row justify-between items-end space-x-4 max-sm:ml-4 sm:ml-6 max-sm:mr-4 sm:mr-6 transition-all duration-500"
                            class:pt-2={banner}
                            class:pb-2={banner}
                        >
                            <h1
                                class="text-white text-3xl font-extrabold mb-1 [text-shadow:_2px_2px_0_rgb(0_0_0_/_20%)]"
                            >
                                {$currentTask?.id ? $currentTask.title : 'All tasks done for now!'}
                            </h1>
                            <h2
                                class="text-white text-3xl font-light mb-1 [text-shadow:_2px_2px_0_rgb(0_0_0_/_20%)]"
                            >
                                {$currentTask?.id ? '12:34' : ''}
                            </h2>
                        </div>
                    </div>
                </div>
            </a>
        {:else}
            <div
                class={banner
                    ? 'flex flex-col bg-gradient-to-tr from-sky-translucent to-emerald-translucent items-center justify-center p-4 max-lg:w-full lg:w-full transition-all duration-500'
                    : 'flex flex-col bg-gradient-to-tr from-sky-500 to-emerald-500 items-center justify-center rounded-3xl p-4 max-lg:w-full lg:w-5/6 transition-all duration-500'}
            >
                <div class="w-full max-w-7xl">
                    <div
                        class="flex flex-row justify-between items-end space-x-4 max-sm:ml-4 sm:ml-6 max-sm:mr-4 sm:mr-6 transition-all duration-500"
                        class:pt-2={banner}
                        class:pb-2={banner}
                    >
                        <h1
                            class="text-white text-3xl font-extrabold mb-1 [text-shadow:_2px_2px_0_rgb(0_0_0_/_20%)]"
                        >
                            {$currentTask?.id ? $currentTask.title : 'All tasks done for now!'}
                        </h1>
                        <h2
                            class="text-white text-3xl font-light mb-1 [text-shadow:_2px_2px_0_rgb(0_0_0_/_20%)]"
                        >
                            {$currentTask?.id ? '12:34' : ''}
                        </h2>
                    </div>
                </div>

                {#if $currentTask?.content}
                    <div
                        class="bg-white w-full text-gray-800 text-lg rounded-2xl mt-3 max-sm:p-4 sm:p-6"
                        transition:slide
                    >
                        {$currentTask?.content || ''}
                    </div>
                {/if}
            </div>
        {/if}
    </div>
</div>
