<script lang="ts">

	import Button from "../atoms/Button.svelte";

</script>

<div class="flex flex-col space-y-10 xl:w-64 lg:w-48 sm:w-32">
    <h1 class="headline-small">
        {$$props.plan.label}
    </h1>
    <h1 class="headline-large">
        ${$$props.plan.cost}
    </h1>
    <Button>
        <p class="body-small font-ubuntu">
            Get Started
        </p>
    </Button>
    <ul class="space-y-1">
        {#each $$props.plan.features as feature}
        <li data-icon="✔️" class="text-left body-medium">
            {feature}
        </li>
        {/each}
    </ul>
</div>