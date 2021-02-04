<script lang='ts'>
	import './main.css';

	import { c } from './character.ts';

	$: document.title = `${$c.name} - DnDCent`;
</script>

<style>
	@font-face {
		font-family: main;
		src: url("fonts/Montserrat/Montserrat-Regular.ttf");
	}
	@font-face {
		font-family: title;
		src: url("fonts/Rajdhani/Rajdhani-Regular.ttf");
	}

	:global(p) {
		font-family: main, sans-serif;
	}

	:global(h1,h2,h3,h4,h5,h6) {
		font-family: title, sans-serif;
	}
</style>

<div>
	<h1 class="">Hello there, {$c.name}</h1>
</div>
