<script lang="ts">
	import Button from '../atoms/Button.svelte';
</script>

<div class="flex flex-col space-y-10 p-7">
	<h1 class="headline-large">
		{$$props.plan.label}
	</h1>
	<div class="flex justify-center flex-row">
		<h1 class="display-medium">
			{$$props.plan.unit}{$$props.plan.cost}
		</h1>
		<p class="body-large italics pt-8">
			{$$props.plan.note}
		</p>
	</div>

	<Button>
		<p class="title-large text-primary-on-light dark:text-primary-on-dark">Get Started</p>
	</Button>
	<ul class="flex flex-col space-y-2">
		{#each $$props.plan.features as feature}
			<li class="whitespace-pre text-left title-medium">
				{'  •  '}
				{feature}
			</li>
		{/each}
	</ul>
</div>
