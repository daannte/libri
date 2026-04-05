<script lang="ts">
	interface Props {
		dir: string;
		onclick: (p: string) => void;
	}

	let { dir, onclick }: Props = $props();
</script>

<div
	class="flex cursor-pointer flex-col justify-center rounded-lg border-2 border-border p-3"
	role="button"
	tabindex="0"
	onclick={() => onclick(dir)}
	onkeydown={(e) => e.key === 'Enter' && onclick(dir)}
>
	<div class="flex items-center justify-between">
		<span class="font-medium">/{dir}</span>
	</div>
</div>
