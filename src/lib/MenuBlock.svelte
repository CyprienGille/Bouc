<script lang="ts">
    export let content = "Placeholder";
    export let OnClickFunc;
</script>

<button
    on:click={OnClickFunc}
    class="focus:bg-gray-900 focus:text-white text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md font-semibold text-xl"
    >{content}</button
>
