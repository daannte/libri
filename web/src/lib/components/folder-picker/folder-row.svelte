<script lang="ts">
	import { type components } from '@shiori/api-client';

	type Directory = components['schemas']['EncodableDirectory'];

	interface Props {
		dir: Directory;
		onclick: (d: Directory) => void;
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
	<span class="font-semibold">{dir.name}</span>
	<span class="text-xs text-muted-foreground">/{dir.path}</span>
</div>
