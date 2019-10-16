import commonjs from 'rollup-plugin-commonjs';
import css from 'rollup-plugin-css-porter';
import html from 'rollup-plugin-bundle-html';
import livereload from 'rollup-plugin-livereload';
import resolve from 'rollup-plugin-node-resolve';
import serve from 'rollup-plugin-serve';
import svelte from 'rollup-plugin-svelte';
import sveltePreprocessor from 'svelte-preprocess';
import { terser } from 'rollup-plugin-terser';
import typescript from 'rollup-plugin-typescript2';
import typescriptCompiler from 'typescript';

module.exports = {
	input: 'src/index.ts',
	output: {
		file: 'dist/index.js',
		format: 'iife',
		sourcemap: true,
	},
	plugins: [
		svelte({
			dev: process.env.NODE_ENV === 'development',
			preprocess: sveltePreprocessor({}),
		}),
		html({
			template: 'src/index.html',
			dest: 'dist',
			filename: 'index.html',
		}),
		css({
			dest: 'dist/index.css',
			raw: false,
		}),
		typescript({ typescript: typescriptCompiler }),
		commonjs({ include: 'node_modules' }),
		resolve(),
	].concat(process.env.NODE_ENV !== 'development' ? [
		terser({ sourcemap: true })
	] : [
		serve({
			contentBase: './dist',
			open: false,
		}),
		livereload({ watch: './src' }),
	]),
};
