import { NextApiRequest, NextApiResponse } from "next"
import { db } from "../../db"

const notion_db_id = "b5acab7862024b21a79caddc8d891d2e";

const addContactRequest = async (req: NextApiRequest, res: NextApiResponse) => {
    if (req.method === "POST") {
        const { name, email, message } = req.body
        const response = await fetch(
			`https://api.notion.com/v1/databases/${notion_db_id}`,
			{
				method: "GET",
				headers: {
					"Notion-Version": "2022-02-22"
				}
			}
		);
        console.log(response);
        const data = await new db.Message({ name, email, message }).save();
        let lead = await db.Lead.findOne({ email });
        if (!lead) {
            lead = await new db.Lead({ name, email, messages: [data] }).save();
        } else {
            lead.messages.push(data);
            await lead.save();
        }
        res.status(200).json(data);
    } else {
        res.status(404).json({ error: { message: 'notFound' } });
    }
}

export default addContactRequest;
