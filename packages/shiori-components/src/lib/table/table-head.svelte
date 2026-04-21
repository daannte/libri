<script lang="ts">
	import { cn, type WithElementRef } from '$lib/utils.js';
	import type { HTMLThAttributes } from 'svelte/elements';
	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLThAttributes> = $props();
</script>

<th
	bind:this={ref}
	data-slot="table-head"
	class={cn(
		'h-10 px-2 font-medium text-foreground [&:has([role=checkbox])]:pr-0 text-left align-middle whitespace-nowrap',
		className
	)}
	{...restProps}
>
	{@render children?.()}
</th>
