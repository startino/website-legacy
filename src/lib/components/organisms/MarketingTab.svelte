<script lang="ts">
	import Button from "../atoms/Button.svelte";
	import PlanCard from "../molecules/PlanCard.svelte";

    export let plans = [
    { 
        label: "Basic",
        cost: 19,
        features: [
           ' Initial Setup',
            'Campaign Setup',
            '24/7 Live data dashboard',
            'Search Engine Optimization',
            'Social Media Marketing',
            '20 000 Ad Impressions',
        ],
		 value: 1,
	},
    { 
        label: "Standard",
        cost: 30,
        features: [
            ' Initial Setup',
            'Campaign Setup',
            '24/7 Live data dashboard',
            'Search Engine Optimization',
            'Social Media Marketing',
            '80 000 Ad Impressions',
            'Regular Meetings: Yearly',
            'Consulting & strategy development',
        ],
    	value: 2,
	},
    {
        label: "Optimised",
        cost: 70,
        features: [
            ' Initial Setup 10-12 Days',
            'Campaign Setup',
            '24/7 Live data dashboard',
            'Search Engine Optimization',
            'Social Media Marketing',
            '120 000 Ad Impressions',
            'Regular Meetings: Quarterly',
            'Consulting & strategy development',
            'Social Media Presence',
            ],
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