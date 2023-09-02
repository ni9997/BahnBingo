<script lang="ts">

</script>

<!-- <div class="items-center justify-center m-4 grid grid-cols-3 grid-rows-3 divide-x divide-y divide-solid divide-red-600">
    {#each [0,1,2,3,4,5,6,7,8] as i}
    <div class="card p-4 grow aspect-square rounded-none">
        TEXT
    </div>
    {/each}
</div> -->

<div class="flex flex-1 m-4 md:max-w-2xl bg-green-600">
    {#each [0,1,2] as i}
    <div class="flex flex-col flex-1">
    {#each [0,1,2] as j}
    <div class="card m-1 p-4 aspect-square border-solid border border-red-600 rounded-none flex">
        <div class="">
            SUB GAME {i}, {j}
        </div>
    </div>
    {/each}
    </div>
    {/each}
</div>