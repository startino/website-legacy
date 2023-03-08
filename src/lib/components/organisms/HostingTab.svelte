<script lang="ts">
     export let plans = [
    { label: "Basic",
		 value: 1,
		},
    { label: "Standard",
		 value: 2,
		},
    { label: "Premium",
		 value: 3,
		},
  ];
</script>

<div class="grid grid-cols-3 space-x-7">
    {#each plans as plan}
    <div class="bg-surface-light dark:bg-surface-dark shadow-lg">
        <h1 class="display-small">
            {plan.label}
        </h1>
    </div>
    {/each}
</div>
