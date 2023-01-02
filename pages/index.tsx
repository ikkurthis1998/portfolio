import type { NextPage } from "next";
import Image from "next/image";
import { ActionButton } from "../components/ActionButton";
import { useTheme } from "next-themes";
import { useEffect, useState } from "react";
import { ContactForm } from "../components/ContactForm";

const Home: NextPage = () => {
	return (
		<div>
			🚧 Current Page is under construction 🚧
			{/* TODO: 
			1. Refactor Control Interface
			2. Add Picture and Description 
			3. Add Blogs - Top 3 - With Hashnode API
			4. Add Projects - Top 3 - With Github API
			5. Add styling
		*/}
		</div>
	);
};

export const MyImage = () => {
	const { theme } = useTheme();
	const image = theme === "dark" ? "/image_color.png" : "/image_color.png";
	const height = theme === "dark" ? 73 : 73;
	return (
		<div className="w-full">
			<Image
				src={image}
				alt="profile_image"
				width={60}
				height={height}
				layout="responsive"
			/>
		</div>
	);
};

export const MyDescription = () => {
	return (
		<div className="dark:text-light">
			<p className="text-center md:text-left">
				<span className="block my-2">
					Hi 🙋‍♂️, I am <b>Sree</b>!
				</span>{" "}
				<b>Engineer</b> specialized in full stack <b>web development</b>.
			</p>
		</div>
	);
};

export default Home;
