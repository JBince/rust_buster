use super::entry::BannerEntry;
use crate::{config::Configuration, AUTHOR, VERSION};
use anyhow::Result;
use console::style;
use std::sync::Arc;

pub struct Banner {
    //The live target
    target: BannerEntry,

    //Accepted status codes
    status_codes: BannerEntry,

    //Number of threads
    threads: BannerEntry,

    //Wordlist to use
    wordlist: BannerEntry,

    //Version of the tool
    version: String,

    //Author of the tool
    author: String,
}

impl Banner {
    pub fn new(config: Arc<Configuration>) -> Self {
        //Define the target
        let target = BannerEntry::new("Target", &config.url);

        //Take selected status codes and convert them to a usable banner entry
        let status_codes = {
            let mut codes = Vec::new();

            for code in &config.status_codes {
                codes.push(code.to_string());
            }

            BannerEntry::new("Status Codes", &format!("[{}]", codes.join(", ")))
        };

        //Take selected threads and convert them to a usable banner entry
        let threads = BannerEntry::new("Threads", &config.threads.to_string());

        //Take selected wordlist and convert it to a usable banner entry
        let wordlist = BannerEntry::new("Wordlist", &config.wordlist);

        Self {
            target,
            status_codes,
            threads,
            wordlist,
            version: VERSION.to_string(),
            author: AUTHOR.to_string(),
        }
    }

    fn header(&self) -> String {
        let artwork = format!(
            //https://onlineasciitools.com/convert-text-to-ascii-art Alligator 2 font
            r#"
      :::::::::  :::    :::  :::::::: :::::::::::       :::::::::  :::    :::  :::::::: ::::::::::: :::::::::: ::::::::: 
     :+:    :+: :+:    :+: :+:    :+:    :+:           :+:    :+: :+:    :+: :+:    :+:    :+:     :+:        :+:    :+: 
    +:+    +:+ +:+    +:+ +:+           +:+           +:+    +:+ +:+    +:+ +:+           +:+     +:+        +:+    +:+  
   +#++:++#:  +#+    +:+ +#++:++#++    +#+           +#++:++#+  +#+    +:+ +#++:++#++    +#+     +#++:++#   +#++:++#:    
  +#+    +#+ +#+    +#+        +#+    +#+           +#+    +#+ +#+    +#+        +#+    +#+     +#+        +#+    +#+    
 #+#    #+# #+#    #+# #+#    #+#    #+#           #+#    #+# #+#    #+# #+#    #+#    #+#     #+#        #+#    #+#     
###    ###  ########   ########     ###           #########   ########   ########     ###     ########## ###    ###                                                                                                      

ver: {} author: {}
"#,
            self.version, self.author
        );
        let top = "───────────────────────────┬──────────────────────";
        format!("{artwork}\n{top}")
    }
    fn footer(&self) -> String {
        let bottom = "───────────────────────────┴──────────────────────";
        let instructions = format!(
            "\nPress {} to stop the scan at any time.\n",
            style("CTRL+C").yellow()
        );
        let extra_line = "──────────────────────────────────────────────────";
        let column_headers = format!(
            "{:\u{0020}<width$}{:\u{0020}<width$}{:\u{0020}<width$}{:\u{0020}<width$}{:\u{0020}<width$}\n",
            style("Request").bold(),
            style("Status").bold(),
            style("Method").bold(),
            style("Length").bold(),
            style("Endpoint").bold(),
            width = 15
        );
        format!("{bottom}\n{instructions}\n{extra_line}\n{column_headers}")
    }
    pub fn print_to<W>(&self, mut writer: W) -> Result<()>
    where
        W: std::io::Write,
    {
        writeln!(&mut writer, "{}", self.header())?;
        writeln!(&mut writer, "{}", self.target)?;
        writeln!(&mut writer, "{}", self.status_codes)?;
        writeln!(&mut writer, "{}", self.threads)?;
        writeln!(&mut writer, "{}", self.wordlist)?;
        writeln!(&mut writer, "{}", self.footer())?;
        Ok(())
    }
}
