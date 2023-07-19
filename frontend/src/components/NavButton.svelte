<script lang="ts">
    export let title: string;
    export let href: string = '';
    export let onClick: any = null;
</script>

<a
    {href}
    on:click={onClick}
    class="bg-opacity-10 hover:bg-opacity-20 bg-gray-200 p-3 w-full rounded-md text-gray-300 hover:text-gray-100"
>
    {title}
</a>
