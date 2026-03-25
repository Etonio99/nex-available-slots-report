Welcome to the Nex Analytics tool! This tool is used to obtain analytics for practices that use NexHealth.

### API Keys and Data Access

To start with this tool, you will need an API key. An API key is a code that is used to authenticate yourself with the NexHealth API. You can obtain an API key signing up for the NexHealth API at [developers.nexhealth.com/signup](https://developers.nexhealth.com/signup). You may want to check if your organization already has an API key that you can use.

When you receive an API key, you won't immediately be able to use it to access any practice. Instead, access to the practices you want analytics for will need to be granted. In order to do so, an administrator from the practice can email developers@nexhealth.com authorizing you access to their practice data. Their email can follow this template:

> We authorize {your organization name} to access our practice data through the NexHealth Synchronizer. Our practice address is {enter practice address}.

Once the NexHealth support team receives this message and has confirmed authenticity, access will be granted.

### Using Nex Analytics

Once you're API key and access have been attained, you can now download analytics! To do so, revisit the home page and select the report you want to export. Upon selecting a report, you will guided through the steps needed to receive the necessary data.

### Subdomains

While exporting a report, you may see that you are asked for a practice's "subdomain". This is a formatted version of a practice's name used when programming. The subdomain is often the same as the name of the practice, but with no lowercase letters and dashes instead of spaces (for example, "My Dental Office" turns into "my-dental-office").

To find a subdomain, you can log into [developers.nexhealth.com](https://developers.nexhealth.com/) and search for the practice in the top-left corner. After selecting the practice, you will be able to see the subdomain on their overview page.
