const { Octokit } = require("@octokit/rest");
const github = require("@actions/github");

const octokit = new Octokit({
  auth: process.env.GITHUB_TOKEN,
});

async function processIssue() {

  const context = github.context;
  const issueTitle = context.payload.issue.title;

  // Check if the title of the issue matches the expected format
  if (!issueTitle.startsWith("Add ") || !issueTitle.endsWith(" to contributors")) {
    console.log("This is not a request to add a contributor.");
    return;
  }

  const { owner, repo } = context.repo;
  const issueNumber = context.payload.issue.number;
  const contributorName = issueTitle.slice(4, -16);
  // Regex checks for both markdown and plain text links
  const repoRegex = /Repo: (https?:\/\/github\.com\/[^\s\)]+|\[URL\]\((https:\/\/github\.com\/[^\s\)]+)\))/;


  const issueOpener = context.payload.issue.user.login;
  
  // Check if the contributor name in the title matches the username of the issue opener
  if (contributorName !== issueOpener) {
    let message = `Hey @${issueOpener}, please make sure you're requesting to add your own name in the issue title! 😅`;
    await commentAndTagUser(owner, repo, issueNumber, contributorName, message);
    console.log(`The contributor name "${contributorName}" does not match the issue opener's username "${issueOpener}"`);
    return;
  }
  
  // Check if the issue body contains a link to a GitHub repository
  const repoMatch = context.payload.issue.body.match(repoRegex);
  if (!repoMatch) {
      let message = `Hey @${contributorName}, you need to include a link to your Leo repo in the issue body! 😄`;
      commentAndTagUser(owner, repo, issueNumber, contributorName, message);
      console.error("No repo URL found in the issue body.");
      return;
  }

  const repoURL = (repoMatch[1] || repoMatch[2]).replace(/\)$/,'');
  const [repoName, ownerName] = repoURL.split('/').reverse();

  // Check if the user has starred the Leo repo
  let hasStarred = false;
  
  // Note: This doesn't handle pagination. If they starred the repo more than 100 starred repos ago we will have an issue. 
  try {
    const starredRepos = await octokit.activity.listReposStarredByUser({
        username: issueOpener,
        per_page: 100
    });
    hasStarred = starredRepos.data.some(repo => repo.full_name === 'AleoHQ/leo');

  } catch (error) {
    console.error(`An error occurred while checking starred status: ${error}`);
  }

  if (hasStarred) {
      console.log(`${contributorName} has starred the repo`);
  } else {
      let message = `Hey @${contributorName}, you need to star the [Leo repo](https://github.com/AleoHQ/leo) to be added as a contributor! Go give it a 🌟!`;
      await commentAndTagUser(owner, repo, issueNumber, contributorName, message);
      console.log(`${contributorName} has not starred the repo`)
      return;
  }

  // Extract the requested badge from the issue body.
  const badgeRegex = /Requested badge: (\w+)/;
  const match = context.payload.issue.body.match(badgeRegex);

  if (!match) {
    let message = `Hey @${contributorName}, you need to specify the requested badge in the issue body! 😄`;
    await commentAndTagUser(owner, repo, issueNumber, contributorName, message);
    console.log('Badge not specified in the issue body.');
    return;
  }

  const badgeType = match[1].toLowerCase();
  console.log(`Badge Type: ${badgeType}`);

  const badgeMapping = {
    audio: {emoji: '🔊', title: 'Audio'},
    a11y: {emoji: '♿️', title: 'Accessibility'},
    bug: {emoji: '🐛', title: 'Bug reports'},
    blog: {emoji: '📝', title: 'Blogposts'},
    business: {emoji: '💼', title: 'Business Development'},
    code: {emoji: '💻', title: 'Code'},
    content: {emoji: '🖋', title: 'Content'},
    data: {emoji: '🔣', title: 'Data'},
    doc: {emoji: '📖', title: 'Documentation'},
    design: {emoji: '🎨', title: 'Design'},
    example: {emoji: '💡', title: 'Examples'},
    eventOrganizing: {emoji: '📋', title: 'Event Organizers'},
    financial: {emoji: '💵', title: 'Financial Support'},
    fundingFinding: {emoji: '🔍', title: 'Funding/Grant Finders'},
    ideas: {emoji: '🤔', title: 'Ideas & Planning'},
    infra: {emoji: '🚇', title: 'Infrastructure'},
    maintenance: {emoji: '🚧', title: 'Maintenance'},
    mentoring: {emoji: '🧑‍🏫', title: 'Mentoring'},
    platform: {emoji: '📦', title: 'Packaging'},
    plugin: {emoji: '🔌', title: 'Plugin/utility libraries'},
    projectManagement: {emoji: '📆', title: 'Project Management'},
    promotion: {emoji: '📣', title: 'Promotion'},
    question: {emoji: '💬', title: 'Answering Questions'},
    research: {emoji: '🔬', title: 'Research'},
    review: {emoji: '👀', title: 'Reviewed Pull Requests'},
    security: {emoji: '🛡️', title: 'Security'},
    tool: {emoji: '🔧', title: 'Tools'},
    translation: {emoji: '🌍', title: 'Translation'},
    test: {emoji: '⚠️', title: 'Tests'},
    tutorial: {emoji: '✅', title: 'Tutorials'},
    talk: {emoji: '📢', title: 'Talks'},
    userTesting: {emoji: '📓', title: 'User Testing'},
    video: {emoji: '📹', title: 'Videos'}
  };

  // Check that they are author of the linked repo
  if (ownerName !== contributorName) {
    console.log("owner name", ownerName, "contributor name", contributorName);
    let message = `Hey @${contributorName}, you need to link to your own repo! 😄`;
    await commentAndTagUser(owner, repo, issueNumber, contributorName, message);
    console.log(`The contributor "${contributorName}" does not own the repo "${repoName}"`);
    return;
  }

  // Check if the repo contains a valid Leo application

  console.log("repo name", repoName);
  try {
    await octokit.repos.getContent({
      owner: ownerName,
      repo: repoName,
      path: 'src/main.leo',
    });
    console.log("repo name", repoName);
    console.log(`The repository "${repoName}" under owner "${ownerName}" contains a valid Leo application.`);
  } catch (error) {
    // Check if the error is a 404 and if the message indicates that the repo is not found
    if (error.status === 404 && error.response && error.response.data && error.response.data.message.includes('repo not found')) {
      console.log(`The repository "${repoName}" under owner "${ownerName}" does not exist or is private.`);
      let message = `Hey @${issueOpener}, we could not access the repository you linked. Please ensure the repository exists and is public.`;
      await commentAndTagUser(owner, repo, issueNumber, issueOpener, message);
      return;
    }
    // If main.leo is not found, check for any .leo file
    try {
      const { data } = await octokit.git.getTree({
        owner: ownerName,
        repo: repoName,
        tree_sha: 'HEAD',
        recursive: 'true'
      });
  
      const leoFiles = data.tree.filter(file => file.path.endsWith('.leo'));
  
      if (leoFiles.length > 0) {
        // Found .leo files but not src/main.leo
        console.log(`Found .leo files but not src/main.leo in the repo "${repoName}" under owner "${ownerName}".`);
        let message = `Hey @${contributorName}, we found .leo files in your repo but not main.leo! Consider adding a main.leo file to your repo. 😄`;
        console.log(ownerName, repo, issueNumber, contributorName, message);
        await commentAndTagUser(owner, repo, issueNumber, contributorName, message);
        return;
      } else {
        // No .leo files found at all
        console.log(`No .leo files found in the repo "${repoName}" under owner "${ownerName}".`);
        let message = `Hey @${contributorName}, the repo you linked does not contain a valid Leo application! 😅`;
        await commentAndTagUser(owner, repo, issueNumber, contributorName, message);
        return;
      }
    } catch (error) {
      console.error(`Error searching for .leo files in the repo "${repoName}":`, error);
      return;
    }
  }
  // Fetch README from the GitHub repo
  const { data: readme } = await octokit.repos.getContent({
      owner,
      repo,
      path: 'README.md',
  });

  const readmeContent = Buffer.from(readme.content, 'base64').toString('utf-8');

  async function commentAndTagUser(owner, repo, issueNumber, message) {
    try {
        await octokit.issues.createComment({
            owner,
            repo,
            issue_number: issueNumber,
            body: message,
        });
        console.log("Comment added successfully");
    } catch (error) {
        console.error("Error creating comment:", error);
    }
  }

  async function createPRWithBadge({ owner, repo, updatedReadme, readme, contributorName, badgeType, issueNumber }) {
    // 1. Create a new branch
    const { data: branch } = await octokit.repos.getBranch({
        owner,
        repo,
        branch: 'master'
    });
    const latestCommitSha = branch.commit.sha;
    const branchName = `add-${badgeType}-badge-for-${contributorName}-${Date.now()}`; // Unique branch name
    await octokit.git.createRef({
        owner,
        repo,
        ref: `refs/heads/${branchName}`,
        sha: latestCommitSha
    });

    // 2. Commit the changes to the new branch
    const updatedReadmeBase64 = Buffer.from(updatedReadme).toString('base64');
    await octokit.repos.createOrUpdateFileContents({
        owner,
        repo,
        path: 'README.md',
        message: `Add ${contributorName} to README contributors`,
        content: updatedReadmeBase64,
        sha: readme.sha,
        branch: branchName
    });

    // 3. Open a Pull Request
    const { data: createdPR } = await octokit.pulls.create({
        owner,
        repo,
        title: `Add ${contributorName} to README contributors`,
        head: branchName,
        base: 'master',
        body: `closes #${issueNumber}`
    });

    // 4. Add @AleoHQ/tech-ops as a reviewer
    await octokit.pulls.requestReviewers({
        owner,
        repo,
        pull_number: createdPR.number,
        reviewers: ['AleoHQ/tech-ops']
    });

    console.log(`Created a PR for "${contributorName}" with the "${badgeType}" badge.`);
  }

  // Check if the user's name exists in the README
  const userRegex = new RegExp(contributorName);
  if (userRegex.test(readmeContent)) {
    console.log(`The contributor "${contributorName}" is found in the README.`);

    const badgeCheckRegex = new RegExp(`<a href="https://github.com/${contributorName}/[^"]*" title=["“]${badgeType}(["”])?>`, 'i');
    
    if (badgeCheckRegex.test(readmeContent)) {
      // TODO: commentAndTagUser
      let message = `Hey @${contributorName}, you already have the "${badgeType}" badge! 😄`;
      await commentAndTagUser(owner, repo, issueNumber, contributorName, message);
      console.log(`The contributor "${contributorName}" already has the "${badgeType}" badge.`);
      return;
    } else {
      console.log('Badge not found in the README for the contributor.');
       // Identify the position to insert the new badge for existing contributor
      const insertionRegex = new RegExp(`(https://github.com/${contributorName}/[^"]*"[^>]*>.*?</a>)`);
      const match = readmeContent.match(insertionRegex);
      
      if (match) {
            const insertionPoint = match.index + match[0].length;
            const badgeDetails = badgeMapping[badgeType];

            if (!badgeDetails) {
                let message = `Hey @${contributorName}, the badge type "${badgeType}" is not recognized! 😅`;
                await commentAndTagUser(owner, repo, issueNumber, contributorName, message);
                throw new Error(`Badge type "${badgeType}" not recognized.`);
            }

            // NOTE: Currently adds link to contributor's page if not tutorial or code
            let badgeLink;
            switch(badgeType.toLowerCase()) {
                case 'tutorial':
                    badgeLink = `https://github.com/${contributorName}/${repoName}`;
                    break;
                case 'code':
                    badgeLink = `https://github.com/AleoHQ/leo/commits?author=${contributorName}`;
                    break;
                default:
                    badgeLink = `https://github.com/${contributorName}`;
                    break;
            }

            const newBadge = `<a href="${badgeLink}" title="${badgeDetails.title}">${badgeDetails.emoji}</a>`;

            const updatedReadme = [
                readmeContent.slice(0, insertionPoint),
                newBadge,
                readmeContent.slice(insertionPoint)
            ].join('');

            await createPRWithBadge({
              owner,
              repo,
              updatedReadme,
              readme,
              contributorName,
              badgeType,
              issueNumber
            });
            
            console.log(`Created a PR to add the "${badgeType}" badge for the contributor "${contributorName}" in the README.`);
          } else {
            let message = `Hey @${contributorName}, we had an issue with your request, please reach out 😅`;
            await commentAndTagUser(owner, repo, issueNumber, contributorName, message);
            console.error(`Failed to find an insertion point for the "${badgeType}" badge for "${contributorName}".`);
        }
      }
    } else {
      console.log(`The contributor "${contributorName}" is NOT found in the README.`);
      let contributorCountMatch = readmeContent.match(/Total count contributors: (\d+)/i);
      let currentCount = contributorCountMatch ? parseInt(contributorCountMatch[1]) : 0;

      const badgeDetails = badgeMapping[badgeType];
      if (!badgeDetails) {
          let message = `Hey @${contributorName}, we had an issue with your request, please reach out 😅`;
          await commentAndTagUser(owner, repo, issueNumber, contributorName, message);
          throw new Error(`Badge type "${badgeType}" not recognized.`);
      }

      //  regex that finds where to place the new badge by finding the second to last <tr> that contains <td>s
      const trMatches = [...readmeContent.matchAll(/<tr>\s*([\s\S]*?)<\/tr>/g)];
      const trMatchesContainingTds = trMatches.filter(trMatch => /<td[^>]*>[\s\S]*?<\/td>/g.test(trMatch[1]));
      const secondToLastTrContainingTds = trMatchesContainingTds[trMatchesContainingTds.length - 2];

      if (secondToLastTrContainingTds) {
          const tdMatchesInTr = secondToLastTrContainingTds[1].match(/<td[^>]*>[\s\S]*?<\/td>/g) || [];
          let updatedReadme;
          const newContributorBlock = `
          <td align="center" valign="top" width="14.28%">${newBadge}<img src="https://avatars.githubusercontent.com/${contributorName}?s=80&v=4?s=100" width="100px;" alt="${contributorName}"/><br /><sub><b>${contributorName}</b></sub></a><br /><a href="https://github.com/${contributorName}/YOUR_REPO_NAME" title="${badgeDetails.title}">${badgeDetails.emoji}</a></td>
          `;

          if (tdMatchesInTr.length < 7) {
              // Insert the new contributor in the last row if there are less than 7 contributors in that row
              const lastTdEndIndex = secondToLastTrContainingTds.index + secondToLastTrContainingTds[0].lastIndexOf('</td>') + 5;
              updatedReadme = [
                  readmeContent.slice(0, lastTdEndIndex),
                  newContributorBlock,
                  readmeContent.slice(lastTdEndIndex)
              ].join('');
          } else {
              // Insert a new row with the new contributor if there are already 7 contributors in the last row
              updatedReadme = [
                  readmeContent.slice(0, secondToLastTrContainingTds.index + secondToLastTrContainingTds[0].length),
                  '\n<tr>',
                  newContributorBlock,
                  '</tr>\n',
                  readmeContent.slice(secondToLastTrContainingTds.index + secondToLastTrContainingTds[0].length)
              ].join('');
          }

          // Increment and update contributor count
          currentCount++;
          updatedReadme = updatedReadme.replace(/Total count contributors: \d+/i, `Total count contributors: ${currentCount}`);

          await createPRWithBadge({
            owner,
            repo,
            updatedReadme,
            readme,
            contributorName,
            badgeType,
            issueNumber
        });
          console.log(`Created a PR to add "${contributorName}" to the README contributors with the "${badgeType}" badge.`);
      } else {
          let message = `Hey @${contributorName}, we had an issue with your request, please reach out 😅`;
          await commentAndTagUser(owner, repo, issueNumber, contributorName, message);
          console.error(`Failed to find the insertion point in the README.`);
      }
    }
}

processIssue().catch(error => {
  console.error(error);
  process.exit(1);
});
