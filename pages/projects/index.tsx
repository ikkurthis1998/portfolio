import type { NextPage } from "next";

const Projects: NextPage = () => {
	return (
		<div>
			<ProjectsTitle />
			{/* <ul>
				<li>
					<b>Files, Apr 2022</b>
					<br />
					Microservice to upload and download files.
					<br />
					API Doc:
					https://documenter.getpostman.com/view/15410355/Uyr5oKEi
					<br />
					Code: https://github.com/ikkurthis1998/files
					<br />
				</li>

				<li>
					<b>Authentication, Apr 2022</b>
					<br />
					Microservice for user signin, signup.
					<br />
					Tech Stack: NodeJS, ExpressJS, MongoDB
					<br />
					API Doc:
					https://documenter.getpostman.com/view/15410355/Uyr5neeu
					<br />
					Code: https://github.com/ikkurthis1998/authentication
				</li>

				<li>
					<b>Message, a public group chat app, Sep 2021</b>
					<br />
					A public chat app to chat with anyone online!
					<br />
					Tech Stack: ReactJS, NodeJS, MongoDB, ExpressJS, Socket.io
					<br />
					Live: https://message-r8.netlify.app/
					<br />
					Code: Frontend:
					https://github.com/ikkurthis1998/message-frontend,
					<br />
					Backend: https://github.com/ikkurthis1998/message-backend
				</li>

				<li>
					<b>My Note, an online notes app, Apr 2021 </b>
					<br />
					An app to take notes. Saves everything in the cloud, so you
					don't misplace your notes ever again. <br />
					Tech Stack: ReactJS, Firebase. <br />
					Live: https://mynotes-reactwithfirebase.web.app/
					<br />
					Code: https://github.com/ikkurthis1998/mynote
				</li>

				<li>
					<b>
						Iron Man Art Compilation, a photo gallery of Iron Man
						Art, Apr 2021{" "}
					</b>
					<br />A fun compilation of Iron Man Art. If you have any,
					add them too!
					<br /> Tech Stack: ReactJS, Firebase.
					<br /> Live: https://photogallery-reactwithfirebase.web.app/
					<br />
					Code:
					https://github.com/ikkurthis1998/photo-galleryreact-firebase
				</li>
			</ul> */}
		</div>
	);
};

export default Projects;

function ProjectsTitle() {
	return (
		<h1 className='text-3xl font-bold underline text-center'>
			Projects!
		</h1>
	);
}
