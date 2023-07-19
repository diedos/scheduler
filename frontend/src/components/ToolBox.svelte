<script lang="ts">
    import { enhance } from '$app/forms';
    import { fly, blur, slide } from 'svelte/transition';
</script>

<div
    class="bg-gray-50 text-gray-800 dark:bg-slate-800 flex flex-col rounded-xl dark:text-slate-300 overflow-hidden border w-1/2 p-4 space-y-4"
    in:fly={{ y: 20 }}
    out:slide
>
    <header class="justify-between items-center flex flex-row">
        <h4 class="flex text-xl">Lorem</h4>
    </header>
    <section class="flex flex-1">
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Illo quae quam nisi ad, pariatur
        soluta dolorum odio dolores? Numquam iusto corporis obcaecati labore sunt temporibus eveniet
        delectus? Minus, sunt at!<br /><br />
        Lorem ipsum dolor, sit amet consectetur adipisicing elit. Dolorum similique vero dignissimos
        officiis? Odio explicabo velit quisquam quasi ab minus nostrum eligendi aspernatur repellendus.
        Ad dicta consectetur a ex delectus.
    </section>
</div>
