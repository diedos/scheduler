<script lang="ts">
    import { slide, scale } from 'svelte/transition';

    export let banner = false;
</script>

<div class="transition-all duration-500 max-w-7xl" class:max-w-full={banner} class:w-full={banner}>
    <div
        class="flex flex-col flex-grow lg:flex-row-reverse sm:m-4 justify-center transition-all duration-500"
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
        <div
            class={banner
                ? 'flex flex-col bg-gradient-to-tr from-sky-500 to-emerald-500 items-center justify-center max-lg:rounded-b-3xl p-4 max-lg:w-full lg:w-full transition-all duration-500'
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
                        Lorem ipsum dolor sit amet consectetur, adipisicing elit
                    </h1>
                    <h2
                        class="text-white text-3xl font-light mb-1 [text-shadow:_2px_2px_0_rgb(0_0_0_/_20%)]"
                    >
                        12:34
                    </h2>
                </div>
            </div>

            {#if !banner}
                <div
                    class="bg-white text-gray-800 text-lg rounded-2xl mt-3 max-sm:p-4 sm:p-6"
                    transition:slide
                >
                    Lorem ipsum dolor sit amet consectetur adipisicing elit. Laborum enim, ratione
                    facere consectetur minus pariatur veniam nam! Repudiandae dolorum obcaecati
                    alias neque iure porro sed at veritatis itaque! Corporis, sequi.<br /><br />
                    Lorem ipsum dolor, sit amet consectetur adipisicing elit. Quidem dicta laborum dolorem
                    minus fugit id repellat voluptatibus eum. Necessitatibus, enim quaerat esse consectetur
                    nihil aliquid quo, ipsa harum expedita consequatur facere officiis rerum iste, suscipit
                    sed? Aspernatur porro fuga maxime unde deserunt illo qui, architecto similique blanditiis
                    modi ea quae perferendis, consectetur, laudantium debitis nobis eligendi ipsa culpa
                    enim sed aut quam dolor! Harum odio officia itaque fuga esse iste deleniti ratione
                    ducimus laboriosam tempora voluptas, temporibus deserunt aspernatur assumenda, veniam
                    non labore fugit omnis recusandae. Iste nam deserunt illum porro neque reprehenderit,
                    quae repudiandae facilis suscipit nihil, laborum quasi.
                </div>
            {/if}
        </div>
    </div>
</div>
