import type { NextPage } from 'next';
import Image from 'next/image';
import { ActionButton } from '../components/ActionButton';
import { useTheme } from 'next-themes';
import { useEffect, useState } from 'react';
import { ContactForm } from '../components/ContactForm';

const Home: NextPage = () => {
    return (
		<div className='h-full flex flex-col justify-around overflow-scroll no-scrollbar'>
			<div className='flex flex-col md:flex-row gap-6 gap-x-10 items-center mb-8'>
				<div className='flex justify-center rounded-3xl overflow-hidden w-6/12 lg:w-10/12 lg:mb-8  md:order-2'>
					<MyImage />
				</div>
				<div className='flex flex-col items-center md:items-start gap-10 md:order-1'>
					<MyDescription />
					{/* <ActionButton
						onClick={() => console.log("Know more clicked")}
						className='px-4 py-1'
					>
						Know More
					</ActionButton> */}
				</div>
			</div>
		</div>
	);
}

export const MyImage = () => {
  const { theme } = useTheme();
  const image = theme === 'dark' ? '/image_color.png' : '/image_color.png';
  const height = theme === 'dark' ? 73 : 73;
  return (
		<div className='w-full'>
			<Image
				src={image}
				alt='profile_image'
				width={60}
				height={height}
        layout='responsive'
			/>
		</div>
  );
}

export const MyDescription = () => {
  return (
		<div className='lg:text-xl dark:text-light'>
			<p className='text-center md:text-left'>
				<span className='block my-2'>
					Hi 🙋‍♂️, I am <b>Sree</b>!
				</span>{" "}
				<b>Engineer</b> specialized in full stack{" "}
				<b>web development</b>.
			</p>
		</div>
  );
}

export default Home;
