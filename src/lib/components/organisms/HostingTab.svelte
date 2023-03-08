<script lang="ts">
	import Button from "../atoms/Button.svelte";
	import PlanCard from "../molecules/PlanCard.svelte";


     export let plans = [
    { 
        label: "Basic",
        cost: 29,
        features: ['Hosting'],
		 value: 1,
	},
    { 
        label: "Standard",
        cost: 59,
        features: ['Hosting', 'Priority Customer Suport','1hr Maintenance Work'],
    	value: 2,
	},
    {
        label: "Premium",
        cost: 69,
        features: ['Hosting', 'Priority Customer Suport','8hrs Maintenance Work', ,'8hrs Developement Work'],
		value: 3,
	},
  ];

  export let activeTabValue = 1;
  
  const handleClick = (tabValue: number) => () => (activeTabValue = tabValue);
</script>

<div class="p-3">
    <ul class="flex space-x-10 text-center">
  {#each plans as plan}
      <li class='shadow-xl {activeTabValue === plan.value ? 'active border-2 border-primary-light dark:border-primary-dark transition-all duration-500 scale-110' : 'hover:border-2'}'>
        <button class="inline-block p-4 rounded-t-lg " on:click={handleClick(plan.value)} type="button" role="plan" aria-controls="profile" aria-selected="false">
        <PlanCard plan={plan}/>
        </button>
      </li>
  {/each}
  </ul>
</div>