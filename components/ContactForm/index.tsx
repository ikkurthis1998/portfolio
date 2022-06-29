import { FormEvent, useState } from "react";
import { ActionButton } from "../ActionButton";
import { Input } from "../Input";

export const ContactForm = () => {

    const [name, setName] = useState("");
    const [email, setEmail] = useState("");
    const [message, setMessage] = useState("");
    const [error, setError] = useState("");
    const [success, setSuccess] = useState("");

    const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        if (!name || !email || !message) {
            setError("Please fill all the fields");
            return;
        }
        try {
            const response = await fetch("/api/addContactRequest", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    name,
                    email,
                    message
                })
            });
            if (response.status === 200) {
                setName("");
                setEmail("");
                setMessage("");
                setError("");
                setSuccess("Your request has been submitted successfully");
            } else {
                setError("Something went wrong");
            }
        } catch (e) {
            setError("Something went wrong");
        }
    };

    return (
		<div className='flex flex-col shadow-inner shadow-lg shadow-light/40 p-10 rounded-xl justify-center items-center gap-2 bg-light dark:bg-dark'>
			<h1 className='text-3xl font-bold'>Contact Me</h1>
			<div>
				{/* Error message will be displayed here */}
				{error && <p className='text-red-500'>{error}</p>}
			</div>
			<div>
				{/* Success message will be displayed here */}
				{success && <p className='text-green-500'>{success}</p>}
			</div>
			<form
				className='flex flex-col justify-center gap-4'
				onSubmit={handleSubmit}
			>
				<Input label='Name' value={name} setValue={setName} />
				<Input
					label='Email'
					value={email}
					setValue={setEmail}
					type='email'
				/>
				<Input
					label='Message'
					value={message}
					setValue={setMessage}
					type='textarea'
				/>
				<ActionButton
					className='w-full p-2 bg-blue-500 text-white'
					type='submit'
				>
					Send
				</ActionButton>
			</form>
		</div>
	);
}