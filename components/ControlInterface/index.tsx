import Link from "next/link";
import { FC, useState } from "react";
import { ActionButton } from "../ActionButton";
import { ContactForm } from "../ContactForm";
import { ToggleTheme } from "../ToggleTheme";

export const ControlInterface: FC = ({ children }) => {

	const [ contactFormOpen, setContactFormOpen ] = useState(false);

    // if (process.browser) {
    //     useEffect(() => {
    //         const handleResize = () => {
    //             Router.reload();
    //         }
    //         window.addEventListener("resize", handleResize);
    //         return () => window.removeEventListener("resize", handleResize);
    //     }, []);
    // }
    return (
		<div className='h-screen flex flex-col justify-between items-center dark:text-light'>
			<header className='w-screen md:w-10/12 flex justify-around items-center mt-10'>
				{/* <Link
					to='/'
					className='rounded overflow-hidden h-26 w-26 flex items-center'
				>
					<img src={logo} alt='logo' className='h-36' />
				</Link> */}
				<div className='flex-grow flex'>
					<Link href='/'>
						<h1 className=' font-bold font-text subpixel-antialiased flex-grow text-center cursor-pointer'>
							ISREE
						</h1>
					</Link>
					<div className=''>
						<ToggleTheme />
					</div>
				</div>

				<div className='hidden flex justify-between py-1 lg:flex flex-grow'>
					<div className='flex-grow'>
						<p className='text-center  hover:text-grey'>
							<Link href='/projects'>Projects</Link>
						</p>
					</div>
					<div className='flex-grow'>
						<p className='text-center hover:text-grey'>
							<Link href='https://blog.isree.dev/'>Blogs</Link>
						</p>
					</div>
				</div>
				<div className='flex-grow text-center'>
					{/* <ActionButton
						onClick={() => setContactFormOpen(true)}
						className='px-3 py-1'
					>
						Contact Me
					</ActionButton> */}
					<Link href='https://twitter.com/isree1998'>
						<a>Contact Me</a>
					</Link>
				</div>
			</header>
			{children}
			<footer className='w-screen mb-4'>
				{/* TODO: Replace the text buttons with icons */}
				<div className='flex justify-between py-1 lg:hidden'>
					<div className='flex-grow'>
						<p className='text-center  hover:text-grey'>
							<Link href='/projects'>Projects</Link>
						</p>
					</div>
					<div className='flex-grow'>
						<p className='text-center hover:text-grey'>
							<Link href='https://blog.isree.dev/'>Blogs</Link>
						</p>
					</div>
				</div>

				<p className='hidden lg:flex justify-center gap-32'>
					&copy; {new Date().getFullYear()}
					{/* <ToggleTheme /> */}
				</p>
			</footer>
			{/* {contactFormOpen && (
				// display a modal when the contact form is open which closes when the user clicks outside of it
				<div className='fixed inset-0 overflow-hidden'>
						<ContactForm />
				</div>
			)} */}
		</div>
	);
}