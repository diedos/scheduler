<script lang="ts">
    import { slide, scale } from 'svelte/transition';

    import { currentTask } from '../store';

    export let banner = false;
</script>

<div
    class="transition-all duration-500 max-w-7xl backdrop-blur-md mb-4 w-full mt-96"
    class:max-w-full={banner}
    class:sticky={banner}
    class:top-0={banner}
    class:shadow-md={banner}
    class:mt-0={banner}
>
    <div
        class="flex flex-col w-full lg:flex-row-reverse justify-center transition-all duration-500"
        class:sm:mt-0={banner}
        class:sm:ml-0={banner}
        class:sm:mr-0={banner}
    >
        {#if !banner}
            <div
                class="flex max-lg:flex-row lg:flex-col max-lg:space-x-4 lg:space-y-4 lg:ml-4 max-lg:mb-4 max-sm:mb-2 grow lg:w-1/6 overflow-hidden"
            >
                <div
                    class="text-neutral-500 backdrop-blur-sm bg-neutral-300 bg-opacity-10 rounded-xl max-sm:p-2 sm:p-4 border-2 border-neutral-200 max-lg:w-1/2 lg:w-full"
                >
                    Take a break
                </div>
                <div
                    class="text-neutral-500 backdrop-blur-sm bg-emerald-500 bg-opacity-20 rounded-xl max-sm:p-2 sm:p-4 border-2 border-neutral-300 max-lg:w-1/2 lg:w-full"
                >
                    Done!
                </div>
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
                                {$currentTask?.title || ''}
                            </h1>
                            <h2
                                class="text-white text-3xl font-light mb-1 [text-shadow:_2px_2px_0_rgb(0_0_0_/_20%)]"
                            >
                                12:34
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
                            {$currentTask?.title || ''}
                        </h1>
                        <h2
                            class="text-white text-3xl font-light mb-1 [text-shadow:_2px_2px_0_rgb(0_0_0_/_20%)]"
                        >
                            12:34
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
